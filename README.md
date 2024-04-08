<a name="readme-top"></a>

<div align="center">
   <img src="https://img.shields.io/github/contributors/0-don/privnote.svg?style=for-the-badge" />
   <img src="https://img.shields.io/github/forks/0-don/privnote.svg?style=for-the-badge" />
   <img src="https://img.shields.io/github/stars/0-don/privnote.svg?style=for-the-badge" />
   <img src="https://img.shields.io/github/issues/0-don/privnote.svg?style=for-the-badge" />
</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/0-don/privnote">
    <img src="web/static/privnote-logo.svg" alt="Logo" width="200" height="80">
  </a>

  <h1 align="center">Privnote</h1>

  <p align="center">
    <a href="https://privnote.coding.global">Clearnet View</a>
    ·
    <a href="http://pnotegqaaijd3dpqesxfjplwbxk2z6jnaoha7gqalfwiaajqqtsilcqd.onion/">Tor View</a>
    <br />
    Privnote is a secure, open source and zero javascript note sharing service inspired by PrivNote written in rust & svelte.
    <br />
    <!-- <a href="https://github.com/0-don/privnote"><strong>Explore the docs »</strong></a> -->
    <a href="https://github.com/0-don/privnote/issues">Report Bug</a>
    ·
    <a href="https://github.com/0-don/privnote/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[Privnote](https://github.com/0-don/privnote) is a secure, open-source note sharing service inspired by PrivNote. This project is unique because it's built with Svelte but emits zero JavaScript, ensuring a lightweight and fast user experience. The backend is powered by Axum, a highly performant web application framework.

Here's why you should consider using or contributing to Privnote:

- It's secure: Your notes are safe and can be shared confidently.
- It's efficient: With zero JavaScript emission from Svelte and a performant backend in Axum, Privnote is built for speed.
- It's open-source: This means you can contribute to its development, suggest changes, and help improve it.

Privnote is not just another note sharing service. It's designed to be secure, efficient, and user-friendly.

### Built With

The major frameworks/libraries used to bootstrap this project include:

- [Rust](https://www.rust-lang.org/): A language empowering everyone to build reliable and efficient software.
- [SvelteKit](https://kit.svelte.dev/): A framework for building extremely high-performance web apps.
- [SeaORM](https://www.sea-orm.org/): An async, dynamic and lightweight ORM for Rust.
- [Axum](https://github.com/tokio-rs/axum): A web application framework that ensures high performance.
- [Svelte](https://svelte.dev/): A JavaScript framework for building user interfaces. In this project, it's configured to emit zero JavaScript.
- [PostgreSQL](https://www.postgresql.org/): A powerful, open source object-relational database system.
- [Tailwind CSS](https://tailwindcss.com/): A utility-first CSS framework for rapidly building custom designs.
- [TypeScript](https://www.typescriptlang.org/): A typed superset of JavaScript that compiles to plain JavaScript.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

To get a local copy of Privnote up and running, follow these steps:

### Prerequisites

- You need to have Rust installed. If you don't have it, you can install it from [here](https://www.rust-lang.org/tools/install).
- You need Node.js and npm installed. If you don't have them, you can install Node.js from [here](https://nodejs.org/en/download/) which includes npm.

### Installation

1. Clone the repo
   ```
   git clone https://github.com/0-don/privnote.git
   ```
2. Change directory to the cloned repo
   ```
   cd privnote
   ```
3. Open a new terminal and start the server
   ```
   cd server && cargo run
   ```
4. Open a new terminal and start the Web App
   ```
   yarn
   yarn dev
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Privnote is a secure, open-source note sharing service. You can use it to share notes securely with others. Simply write your note, generate a link, and share it. The note will self-destruct after being read.

## Roadmap

See the [open issues](https://github.com/0-don/privnote/issues) for a list of proposed features and known issues.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Contact

Project Link: [https://github.com/0-don/privnote](https://github.com/0-don/privnote)

## Acknowledgments

- [Svelte](https://svelte.dev/)
- [Axum](https://github.com/tokio-rs/axum)
- [SeaORM](https://www.sea-orm.org/)
- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [PostgreSQL](https://www.postgresql.org/)
- [SvelteKit](https://kit.svelte.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [TypeScript](https://www.typescriptlang.org/)
- [ESLint](https://eslint.org/)
- [Prettier](https://prettier.io/)
<p align="right">(<a href="#readme-top">back to top</a>)</p>
