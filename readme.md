### retrieve information from a SQLite database, with a simple text-based user interface

Retrieve information from an existing database with user-supplied keys.

Database consists of "Bosch" TV show episode titles names. In this case a season-episode number pair is a unique key, there is always just one title with that combination of numbers, so the app logic can be simplified.

When nothing is found, the returned string is empty and an appropriate message appears.

The episodes' titles were obtained from the Wikipedia article with Python requests, BeautifulSoup (HTML parsing) and re(regex) libraries.