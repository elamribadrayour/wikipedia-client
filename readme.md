# Wikipedia API Rust Library

Welcome to the Wikipedia API Rust Library! This library provides an easy and efficient way to interact with Wikipedia's vast knowledge base using Rust. Whether you want to search for articles, retrieve page content, or analyze page statistics, this library has you covered.

## Features

- **Search Wikipedia Articles**: Find pages related to a given search query.
- **Retrieve Page Content**: Fetch detailed content from Wikipedia pages by title or page ID.
- **Access Media Files**: Get a list of media files associated with a Wikipedia page.
- **Explore Categories**: Discover the categories a Wikipedia page belongs to.
- **Follow Links**: Retrieve all links present on a Wikipedia page.
- **Language Support**: Get information about different language versions of a page.
- **View Statistics**: Access the view count statistics for a page over a specified time period.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. If not, you can download and install it from [rust-lang.org](https://www.rust-lang.org/).

### Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
wikipedia_api = "0.1.0"  # Replace with the actual version
```

### Usage

Here is a quick start guide on how to use the library:

#### Searching Articles

```rust
use wikipedia_api::search;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let results = search(query).await?;
    println!("{:?}", results);
    Ok(())
}
```

#### Fetching Page Content by Title

```rust
use wikipedia_api::get_page_by_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let title = "Rust (programming language)";
    let page_content = get_page_by_title(title).await?;
    println!("{:?}", page_content);
    Ok(())
}
```

#### Fetching Page Content by Page ID

```rust
use wikipedia_api::get_page_by_id;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page_id = "123456";
    let page_content = get_page_by_id(page_id).await?;
    println!("{:?}", page_content);
    Ok(())
}
```

#### Fetching Media Files

```rust
use wikipedia_api::get_images;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let images = get_images(query).await?;
    println!("{:?}", images);
    Ok(())
}
```

#### Fetching Categories

```rust
use wikipedia_api::get_categories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let categories = get_categories(query).await?;
    println!("{:?}", categories);
    Ok(())
}
```

#### Fetching Links

```rust
use wikipedia_api::get_links;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let links = get_links(query).await?;
    println!("{:?}", links);
    Ok(())
}
```

#### Fetching Languages

```rust
use wikipedia_api::get_languages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let languages = get_languages(query).await?;
    println!("{:?}", languages);
    Ok(())
}
```

#### Fetching View Statistics

```rust
use chrono::NaiveDate;
use wikipedia_api::get_views;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = "Rust programming";
    let start_date = "2023-01-01";
    let nb_days = 30;
    let views = get_views(query, start_date, nb_days).await?;
    println!("{:?}", views);
    Ok(())
}
```

## Contributing

We welcome contributions! Please feel free to submit pull requests or report issues.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Wikipedia community for their incredible work and the API that makes this library possible.

Enjoy exploring the world of Wikipedia through Rust!