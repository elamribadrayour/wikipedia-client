# Wikipedia Rust Client Library

This library provides a multithreaded asynchronous client for interacting with Wikipedia's API, allowing you to search for pages, retrieve their content, and access various metadata like images, categories, links, languages, and view statistics.

## Features

- Search Wikipedia pages by query.
- Retrieve detailed content of Wikipedia pages.
- Fetch media files associated with Wikipedia pages.
- Access categories and links related to Wikipedia pages.
- Retrieve available languages for a Wikipedia page.
- Get view statistics for a Wikipedia page over a specified period.

## Installation

To use this library, you can add it to your project using the following command:

```bash
cargo add --git https://github.com/elamribadrayour/wikipedia-client.git wikipedia-client
```

Alternatively, if you prefer to add it manually, include the following in your `Cargo.toml`:

```toml
[dependencies]
wikipedia-client = { git = "https://github.com/elamribadrayour/wikipedia-client.git", version = "0.1.0" }
```

## Usage

Here's a quick guide on how to use the library functions. All functions are asynchronous and return results wrapped in a `Result` type, which you should handle appropriately.

### 1. Search Pages

```rust
use wikipedia_client::search;

#[tokio::main]
async fn main() {
    match search("Rust").await {
        Ok(results) => println!("Search results: {:?}", results),
        Err(e) => eprintln!("Error searching pages: {}", e),
    }
}
```

### 2. Get Page Content

```rust
use wikipedia_client::get_content;

#[tokio::main]
async fn main() {
    match get_content("Rust".to_string()).await {
        Ok(contents) => println!("Page contents: {:?}", contents),
        Err(e) => eprintln!("Error retrieving page content: {}", e),
    }
}
```

### 3. Get Images from a Page

```rust
use wikipedia_client::get_images;

#[tokio::main]
async fn main() {
    match get_images("Rust").await {
        Ok(images) => println!("Page images: {:?}", images),
        Err(e) => eprintln!("Error retrieving images: {}", e),
    }
}
```

### 4. Get Categories

```rust
use wikipedia_client::get_categories;

#[tokio::main]
async fn main() {
    match get_categories("Rust").await {
        Ok(categories) => println!("Page categories: {:?}", categories),
        Err(e) => eprintln!("Error retrieving categories: {}", e),
    }
}
```

### 5. Get Links

```rust
use wikipedia_client::get_links;

#[tokio::main]
async fn main() {
    match get_links("Rust").await {
        Ok(links) => println!("Page links: {:?}", links),
        Err(e) => eprintln!("Error retrieving links: {}", e),
    }
}
```

### 6. Get Languages

```rust
use wikipedia_client::get_languages;

#[tokio::main]
async fn main() {
    match get_languages("Rust").await {
        Ok(languages) => println!("Page languages: {:?}", languages),
        Err(e) => eprintln!("Error retrieving languages: {}", e),
    }
}
```

### 7. Get Views

```rust
use wikipedia_client::get_views;
use chrono::NaiveDate;

#[tokio::main]
async fn main() {
    match get_views("Rust", "2023-01-01", 30).await {
        Ok(views) => println!("Page views: {:?}", views),
        Err(e) => eprintln!("Error retrieving views: {}", e),
    }
}
```

## Contributing

Contributions to the library are welcome. Please submit a pull request or open an issue for any bugs or feature requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
