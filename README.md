<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/jeffersonroth">
    <img src="https://raw.githubusercontent.com/jeffersonroth/common-assets/main/assets/images/logo.svg" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">mdBook Reference Table Preprocessor</h3>

  <p align="center">
    mdBook preprocessor to create Reference Tables.
  </p>
</p>

<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- GETTING STARTED -->

## Getting Started

1. Install the cli tool:

   ```sh
   cargo install mdbook-reference-table
   ```

2. Add the preprocessor to your `book.toml` file:

   ```toml
   [preprocessor.reference-table]
    reference-table = "./reference-table.yaml" # Relative to your book.toml
   ```

3. Consult and edit your reference table file `reference-table.yaml`. Example:
   
   ```yaml
   reference-table:
    - id: "audita-quot"
      path: "content/subject-1/audita-quot.md"
    - id: "bos-quae-inde-limen"
      path: "content/subject-2/subfolder-1/bos-quae-inde-limen.md"
    - id: "esse-corpora-achaidas-sacros-ad-quas"
      path: "content/subject-3/subfolder-1/subsubfolder-1/esse-corpora-achaidas-sacros-ad-quas.md"
   ```

4. Add references:

   ```markdown
   <!-- content/subject-1/audita-quot.md -->

    - Self reference: {{reference: {id: "audita-quot", title: "Audit quot"}}}.
    - Same level reference: {{reference: {id: "aliter-facta-cornibus-tandem", title: "Aliter facta cornibus tandem"}}}.
    - Deep nested reference: {{reference: {id: "bos-quae-inde-limen", title: "Bos quae inde lime"}}}.
    - Deeper nested reference: {{reference: {id: "esse-corpora-achaidas-sacros-ad-quas", title: "Esse corpora Achaidas sacros ad quas"}}}.
   ```

5. Build your book and serve it locally:

   ```sh
   mdbook serve --hostname 0.0.0.0
   ```

6. Verify the rendered links are correct.

<!-- LICENSE -->

## License

Copyright (C) 2024 Jefferson Johannes Roth Filho. See `LICENSE` for more information.

<!-- CONTACT -->

## Contact

Jefferson Roth - <jjrothfilho@gmail.com>

Project Link: [https://hub.docker.com/r/jeffroth/mdbook-reference-table](https://hub.docker.com/r/jeffroth/mdbook-reference-table)

crates.io: [mdbook-reference-table](https://crates.io/crates/mdbook-reference-table)
