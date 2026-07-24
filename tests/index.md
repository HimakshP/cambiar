HTML to Markdown Test Suite

# Header 1 (H1)

## Header 2 (H2)

### Header 3 (H3)

#### Header 4 (H4)

##### Header 5 (H5)

###### Header 6 (H6)

This paragraph tests **bold text via strong** and **bold text via b tags**.

This paragraph tests *italic text via em* and *italic text via i tags*.

This paragraph tests strikethrough text via del and strikethrough via s tags.

This is ***combined bold and italic*** text.

Here is a standard [hyperlink to Example](https://example.com).

Here is an image with alt text:

![A beautiful placeholder image](https://example.com)

### Unordered List

*   First item
*   Second item

### Ordered List

1.  Step one
2.  Step two

### Nested List Hierarchy

*   Top level item A
    1.  Nested ordered item 1
    2.  Nested ordered item 2
*   Top level item B
    *   Nested unordered item 1

Use the `console.log()` method to print a message to the debugging console.

```
function greetUser(name) {
    return `Hello, ${name}!`;
}
console.log(greetUser("World"));
```

> This is a single-line blockquote.

> This is a multi-line blockquote.
> 
> It should cleanly generate multiple greater-than signs if nested properly.

| Product    | Price  | Status       |
| ---------- | ------ | ------------ |
| Item Alpha | $10.00 | In Stock     |
| Item Beta  | $25.00 | Out of Stock |

* * *

The line above is a horizontal rule. The following contains a line break:  
This text is on a new line.