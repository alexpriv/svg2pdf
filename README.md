# svg2pdf

`svg2pdf` is a CLI that allows you to join multiple SVGs, turning them into PDFs. Internally, it:

1. reads the provided SVGs;
2. wraps the SVGs in a HTML `div` tag with the `height` CSS attribute set to `100vh`;
3. appends their content to a final HTML string;
4. uses `wkhtmltopdf` to convert the HTML into a PDF with QtWebKit; and
5. writes the output to either stdout or a provided file.

## Installation

See [the releases](https://github.com/alexpriv/svg2pdf/releases). [`wkhtmltopdf`](https://wkhtmltopdf.org) is also required.

## Usage

Run `svg2pdf --help` to get a reference.

### Getting the raw output

To combine two SVGs, convert them into a PDF and output the final result, don't pass the `--out` flag:

```console
$ svg2pdf page1.svg page2.svg
<raw file content>
```

This doesn't save a file.

### Saving the output to a file

Use the `--out` flag:

```console
$ svg2pdf page1.svg page2.svg --out test.pdf
```

This should create a file called `test.pdf` with the output.

### Manipulating the document

There's a few config flags that you're able to use.

#### --landscape

This will make the document landscape.

```console
$ svg2pdf page1.svg page2.svg --landscape
```

#### --margin

This will set the document's margin in either inches or millimetres. Inches will be used unless the `--metric` flag is passed.

```console
$ svg2pdf page1.svg page2.svg --margin 2
```

#### --metric

Sets the unit of measurement to millimetres. Note that values must be round numbers.

```console
$ svg2pdf page1.svg page2.svg --metric --margin 51
```

#### -v, -vv

If you pass `-v`, debug logging will be enabled. Passing `-vv` will enable debug _and_ trace logging.

## Example Implementations

### PHP

```php
exec(escapeshellcmd("svg2pdf page1.svg page2.svg --landscape --margin 2"), $raw_output);
$parsed_output = join("\n", $raw_output);
// $parsed_output => <raw PDF content>

exec(escapeshellcmd("svg2pdf page1.svg page2.svg --landscape --margin 2 --out combined.pdf"));
// combined.pdf is the final PDF.
```
