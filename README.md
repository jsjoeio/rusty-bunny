# Rusty Bunny

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/jsjoeio/rusty-bunny">
    <img src="logo.png" alt="Logo" width="64" height="64">
  </a>

  <h3 align="center">rusty-bunny</h3>

  <p align="center">
    rusty-bunny is a mini-clone of <a href="http://www.bunny1.org/">bunny1  </a>
    <br />
    "a tool that lets you write smart bookmarks in [rust] and then share them across all your browsers..."
    <br />
    <a href="https://github.com/jsjoeio/rusty-bunny"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/jsjoeio/rusty-bunny#demo">View Demo</a>
    ·
    <a href="https://github.com/jsjoeio/rusty-bunny/issues">Report Bug</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
## Table of Contents

- [Rusty Bunny](#rusty-bunny)
  - [Table of Contents](#table-of-contents)
  - [About the Project](#about-the-project)
  - [Demo](#demo)
    - [Built With](#built-with)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)
  - [Contact](#contact)
  - [Acknowledgements](#acknowledgements)

<!-- ABOUT THE PROJECT -->
## About the Project

The idea for this project came after learning about `bunny1` and using it at work. I really enjoyed it and thought, "I wonder if I could build my own from scratch?"

As part of my [Ultralearning project for Rust](https://joeprevite.com/rust-learning-plan-chapter-1-notes), I decided to take myself up on the challenge. This project is the result of that. 

## Demo

![rusty-bunny demo][product-screenshot]

This is what `rusty-bunny` looks like in action.

### Built With

* [Rust](https://www.rust-lang.org/)
* [Rocket](https://rocket.rs/)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

Make sure you have Rust installed. 

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rocket uses the nightly version of Rust so make sure you use that. If you'd like to only use nightly for this project, you can run this from the root of the project after cloning.

```sh
# from the root of the project
rustup override set nightly
```

### Installation
 
1. Clone the rusty-bunny
```sh
git clone https://github.com/jsjoeio/rusty-bunny.git
```
2. Make sure you're using nightly 
```sh
cargo --version
```
3. Build the project
```sh
cargo build
```
4. Run the project
```sh
cargo run
```
5. Visit [localhost:8000](http://localhost:8000/)
6. To test a command, go to [localhost:8000/search?cmd=tw](http://localhost:8000/search?cmd=tw) and you should be redirected to Twitter

<!-- USAGE EXAMPLES -->
## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by your command.

The following commands are supported by `rusty-bunny`:
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username
- "gh" -> redirects to github.com
- "gh username" -> redirects to github.com/username
- "gh username/repo" -> redirects to github.com/username/repo
- "mail" -> redirects to mail.google.com
- "cal" -> redirects to calendar.google.com

Everything else redirects to a google search with your query.

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

<!-- LICENSE -->
## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

<!-- CONTACT -->
## Contact

If you have questions or thoughts on this project, feel free to send them my way by @'ing me on Twitter or shooting me a DM.

Joe Previte - [@jsjoeio](https://twitter.com/jsjoeio)

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [The Rust Community](https://www.rust-lang.org/community)
* [Rocket.rs](https://rocket.rs/)
* [@othneildrew](https://github.com/othneildrew) - for the [README template](https://github.com/othneildrew/Best-README-Template)
* [@tscritch](https://github.com/tscritch) - for the code suggestions

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/jsjoeio/rusty-bunny.svg?style=flat-square
[contributors-url]: https://github.com/jsjoeio/rusty-bunny/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/jsjoeio/rusty-bunny.svg?style=flat-square
[forks-url]: https://github.com/jsjoeio/rusty-bunny/network/members
[stars-shield]: https://img.shields.io/github/stars/jsjoeio/rusty-bunny.svg?style=flat-square
[stars-url]: https://github.com/jsjoeio/rusty-bunny/stargazers
[issues-shield]: https://img.shields.io/github/issues/jsjoeio/rusty-bunny.svg?style=flat-square
[issues-url]: https://github.com/jsjoeio/rusty-bunny/issues
[license-shield]: https://img.shields.io/github/license/jsjoeio/rusty-bunny?style=flat-square
[license-url]: https://github.com/jsjoeio/rusty-bunny/blob/master/LICENSE
[product-screenshot]: demo.gif
