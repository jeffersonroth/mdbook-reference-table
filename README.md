<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/jeffersonroth/mdbook-environment">
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

   Note: not yet published to crates.io. In the meantime you can clone this repository.

2. Add the preprocessor to your `book.toml` file:

   ```toml
   [preprocessor.reference-table]
    command = "../../target/release/mdbook-reference-table.exe" # If Windows
    reference-table = "./reference-table.yaml" # Relative to your book.toml
   ```

3. Build your book and serve it.

4. Consult and edit your reference table file `reference-table.yaml`. Example:
   
   ```yaml
   reference-table:
    - id: "audita-quot"
      path: "content/subject-1/audita-quot.md"
    - id: "bos-quae-inde-limen"
      path: "content/subject-2/subfolder-1/bos-quae-inde-limen.md"
    - id: "esse-corpora-achaidas-sacros-ad-quas"
      path: "content/subject-3/subfolder-1/subsubfolder-1/esse-corpora-achaidas-sacros-ad-quas.md"
   ```

<!-- LICENSE -->

## License

Copyright (C) 2024 Jefferson Johannes Roth Filho. See `LICENSE` for more information.

<!-- CONTACT -->

## Contact

Jefferson Roth - <jjrothfilho@gmail.com>

Project Link: [https://hub.docker.com/r/jeffroth/mdbook-reference-table](https://hub.docker.com/r/jeffroth/mdbook-reference-table)
