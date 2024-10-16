# Wikipedia API:

1. **Search for Pages:**
   - Find pages related to a keyword or topic.
   - Example API call: `action=query&list=search&srsearch={query}`.
   - Useful for looking up articles, and retrieving page titles, page IDs, and snippets.

2. **Retrieve Page Content:**
   - Get the full content of a Wikipedia page in plain text or HTML.
   - Example API call: `action=query&prop=extracts&titles={title}`.
   - Useful for getting introductory sections or summaries of an article.

3. **Get Page Metadata:**
   - Fetch metadata about a specific page (such as page revisions, contributors, and categories).
   - Example API call: `action=query&prop=info&titles={title}`.
   - Useful for knowing when a page was last updated or its contributors.

4. **Retrieve Media Files:**
   - Fetch images or other media files attached to a page.
   - Example API call: `action=query&prop=images&titles={title}`.
   - Useful for retrieving media or files associated with a specific article.

5. **Search Categories:**
   - Retrieve the categories a specific page belongs to.
   - Example API call: `action=query&prop=categories&titles={title}`.
   - Useful for understanding the classification of an article within Wikipedia's hierarchy.

6. **Page Links:**
   - Get all internal links on a page.
   - Example API call: `action=query&prop=links&titles={title}`.
   - Useful for exploring related topics or navigating through Wikipedia pages programmatically.

7. **Fetch Revisions:**
   - Retrieve the revision history of a page.
   - Example API call: `action=query&prop=revisions&titles={title}`.
   - Useful for understanding changes made over time to an article.

8. **Random Pages:**
   - Get a random Wikipedia page.
   - Example API call: `action=query&list=random&rnlimit=1`.
   - Useful for discovering random articles.

9. **Get Languages:**
   - Retrieve the available languages for a specific page.
   - Example API call: `action=query&prop=langlinks&titles={title}`.
   - Useful for finding language translations of a specific page.

10. **Get Page Views:**
    - Retrieve the page view statistics for a given article.
    - Example API call: `action=query&prop=pageviews&titles={title}`.
    - Useful for tracking popularity or trends of an article over time.
