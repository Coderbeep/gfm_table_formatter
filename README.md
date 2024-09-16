# GFM Table Formatter
Provides a tool for formatting and aligning Markdown tables according to the GFM specification. The spec for the Github Flavored Markdown can be found [here](https://github.github.com/gfm/). It operates on clean text input, rather than parsing a syntax tree, since its purpose it to clean up the formatting for easier visual inspection.

### Example
```md
| Product Name|Price|  Quantity   | Available From|
|:--------|:----|          ----:|:----------------:|
|Super Widget|$12.99 | 100| April 1,2024|
| Mega Widget   |$129.00 |25   |May 15,2024|
|Ultra Widget Pro| $99.5| 5|March 28,2024   |
|Basic Widget| $5.75  |500 | Q1 2024   |
| Premium Widget|   $150.00|0|Not available|
```

```md
| Product Name     | Price   | Quantity | Available From |
|:-----------------|:--------|---------:|:--------------:|
| Super Widget     | $12.99  |      100 |  April 1,2024  |
| Mega Widget      | $129.00 |       25 |  May 15,2024   |
| Ultra Widget Pro | $99.5   |        5 | March 28,2024  |
| Basic Widget     | $5.75   |      500 |    Q1 2024     |
| Premium Widget   | $150.00 |        0 | Not available  |
```