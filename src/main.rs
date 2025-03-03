use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn generate_html(
    template_path: &str,
    output_path: &str,
    values: HashMap<&str, &str>,
) -> io::Result<()> {
    // Read the template file
    let template = fs::read_to_string(template_path)?;

    // Replace placeholders with actual values
    let mut output = template;
    for (key, value) in values {
        let placeholder = format!("{{{{{}}}}}", key);
        output = output.replace(&placeholder, value);
    }

    // Write the generated HTML to the output file
    let mut file = fs::File::create(output_path)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <template_path> <output_path>", args[0]);
        return;
    }

    let template_path = &args[1];
    let output_path = &args[2];

    // Define the values to replace in the template
    let mut values = HashMap::new();
    values.insert("COMPANY_NAME", "Example Corp");
    values.insert("INVOICE_NUMBER", "12345");
    values.insert("MONTH_ISSUE", "January");
    values.insert("DAY_NUMBER_ISSUE", "1");
    values.insert("YEAR_ISSUE", "2023");
    values.insert("MONTH_DUE", "February");
    values.insert("DAY_NUMBER_DUE", "1");
    values.insert("YEAR_DUE", "2023");
    values.insert("BILL_AUTHOR_NAME", "John Doe");
    values.insert("BILL_AUTHOR_ADDRESS_LINE_1", "123 Main St");
    values.insert("BILL_AUTHOR_ADDRESS_LINE_2", "Suite 100");
    values.insert("BILL_AUTHOR_COUNTRY", "USA");
    values.insert("BILL_AUTHOR_PHONE_NUMBER", "555-1234");
    values.insert("BILL_AUTHOR_EMAIL", "john.doe@example.com");
    values.insert("BILL_TO_NAME", "Jane Smith");
    values.insert("BILL_TO_ADDRESS_LINE_1", "456 Elm St");
    values.insert("BILL_TO_ADDRESS_LINE_2", "Apt 2B");
    values.insert("BILL_TO_PHONE_NUMBER", "555-5678");
    values.insert("BILL_TO_EMAIL", "jane.smith@example.com");
    values.insert("CURRENCY", "$");
    values.insert("AMOUNT_DUE", "100.00");
    values.insert("PAYMENT_LINK", "http://example.com/pay");
    values.insert("PAYMENT_LINK_TEXT", "Pay Now");
    values.insert("PAYMENT_INSTRUCTIONS", "Please pay by the due date.");
    values.insert("LINE_ITEM_DESCRIPTION", "Service Fee");
    values.insert("LINE_ITEM_QUANTITY", "1");
    values.insert("LINE_ITEM_PRICE", "100.00");
    values.insert("LINE_ITEM_AMOUNT", "100.00");
    values.insert("INVOICE_NOTE", "Thank you for your business.");
    values.insert("SUBTOTAL", "100.00");
    values.insert("TOTAL", "100.00");
    values.insert("FOOT_NOTE", "This is a computer-generated invoice.");
    values.insert("CURRENT_PAGE", "1");
    values.insert("MAX_PAGES", "1");

    // Measure the time taken to generate the HTML file
    let start = Instant::now();
    if let Err(e) = generate_html(template_path, output_path, values) {
        eprintln!("Failed to generate HTML: {:?}", e);
    }
    let duration = start.elapsed();
    println!("HTML generation took: {} ms", duration.as_millis());
}
