//! Library usage example of the Memista library
//!
//! This example demonstrates how to use Memista's core functionality directly
//! without starting the HTTP server.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating Memista library usage...");
    
    // Show how to load or create an index (this would require a real database pool in practice)
    println!("Example of loading or creating an index:");
    println!("  let index = load_or_create_index(\"example_db\")?;");
    
    // Show how to ensure a table exists (this would require a real database pool in practice)
    println!("Example of ensuring a table exists:");
    println!("  ensure_table_exists(&db_pool, \"example_db\").await?;");
    
    println!("\nIn practice, you would need to set up a database connection pool");
    println!("and use the actual functions from the memista library.");
    
    println!("\nLibrary usage example completed successfully!");
    Ok(())
}