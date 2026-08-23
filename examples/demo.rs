use oxide_cg::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Define Category Model
    let category_schema = ModelSchema::new("Category")
        .category("E-Commerce")
        .icon("tag")
        .description("Product Categories and Taxonomies")
        .field(Field::string("name").required().unique().searchable())
        .field(Field::string("slug").unique().searchable())
        .field(Field::string("description"))
        .field(Field::boolean("is_active").default_value(serde_json::json!(true)))
        .with_timestamps();

    // 2. Define Product Model (with Money, Progress Bar, Approval on discount)
    let product_schema = ModelSchema::new("Product")
        .category("E-Commerce")
        .icon("shopping-bag")
        .description("Inventory and Catalog Items")
        .field(Field::string("title").required().searchable())
        .field(Field::string("sku").required().unique().searchable())
        .field(Field::money("price", "USD").required().filterable(true))
        // Sensitive field: Changing discount requires manager/admin approval
        .field(Field::float("discount_percent").requires_approval().help("Changes require Manager review"))
        .field(Field::progress_bar("stock_quantity", 500.0, "#22c55e").filterable(true))
        .field(Field::html("description").help("Rich text / Markdown formatted product overview"))
        .field(Field::r#enum("status", vec!["Draft", "Published", "Archived"]))
        .field(Field::foreign_key("category_id", "Category").help("Associated category ID"))
        .field(Field::boolean("is_featured").default_value(serde_json::json!(false)))
        .with_timestamps();

    // 3. Define Customer Order Model
    let order_schema = ModelSchema::new("Order")
        .category("Sales & CRM")
        .icon("credit-card")
        .description("Customer Purchases and Payment Records")
        .field(Field::string("order_number").required().unique().searchable())
        .field(Field::email("customer_email").required().searchable())
        .field(Field::money("total_amount", "USD").required().filterable(true))
        .field(Field::r#enum("payment_status", vec!["Pending", "Paid", "Refunded", "Failed"]))
        .field(Field::r#enum("fulfillment_status", vec!["Unfulfilled", "Processing", "Shipped", "Delivered"]))
        .field(Field::string("shipping_address"))
        .with_timestamps();

    // 4. Define Support Ticket Model
    let ticket_schema = ModelSchema::new("Ticket")
        .category("Support")
        .icon("life-buoy")
        .description("Customer Inquiries and Helpdesk")
        .field(Field::string("subject").required().searchable())
        .field(Field::email("user_email").required().searchable())
        .field(Field::r#enum("priority", vec!["Low", "Medium", "High", "Critical"]))
        .field(Field::r#enum("status", vec!["Open", "In_Progress", "Resolved", "Closed"]))
        .field(Field::markdown("message").required())
        .with_timestamps();

    // 5. Build and launch Oxide_CG with React Ecosystem!
    OxideCGApp::new()
        .site_name("AcroStore Admin")
        .bind("0.0.0.0:8080")
        .database("sqlite://acrostore_cg.db?mode=rwc")
        .register(category_schema)
        .register(product_schema)
        .register(order_schema)
        .register(ticket_schema)
        .run()
        .await?;

    Ok(())
}
