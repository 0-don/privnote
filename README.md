<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/don-cryptus/privnote">
    <img src="web/static/favicon.ico" alt="Logo" width="80" height="80">
  </a>

  <h1 align="center">Privnote</h1>

  <p align="center">
    <a href="https://privnote.coding.global">Clearnet View</a>
    ·
    <a href="http://pnotegqaaijd3dpqesxfjplwbxk2z6jnaoha7gqalfwiaajqqtsilcqd.onion/">Tor View</a>
    <br />
    Privnote is a secure, open source and zero javascript note sharing service inspired by PrivNote written in rust & svelte.
    <br />
    <!-- <a href="https://github.com/don-cryptus/privnote"><strong>Explore the docs »</strong></a> -->
    <a href="https://github.com/don-cryptus/privnote/issues">Report Bug</a>
    ·
    <a href="https://github.com/don-cryptus/privnote/issues">Request Feature</a>
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
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

[Privnote](https://github.com/don-cryptus/privnote) is a secure, open-source note sharing service inspired by PrivNote. This project is unique because it's built with Svelte but emits zero JavaScript, ensuring a lightweight and fast user experience. The backend is powered by Axum, a highly performant web application framework.

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
   git clone https://github.com/don-cryptus/privnote.git
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

See the [open issues](https://github.com/don-cryptus/privnote/issues) for a list of proposed features and known issues.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - [@your_twitter](https://twitter.com/your_username) - email@example.com

Project Link: [https://github.com/don-cryptus/privnote](https://github.com/don-cryptus/privnote)

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

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/don-cryptus/privnote.svg?style=for-the-badge
[contributors-url]: https://github.com/don-cryptus/privnote/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/don-cryptus/privnote.svg?style=for-the-badge
[forks-url]: https://github.com/don-cryptus/privnote/network/members
[stars-shield]: https://img.shields.io/github/stars/don-cryptus/privnote.svg?style=for-the-badge
[stars-url]: https://github.com/don-cryptus/privnote/stargazers
[issues-shield]: https://img.shields.io/github/issues/don-cryptus/privnote.svg?style=for-the-badge
[issues-url]: https://github.com/don-cryptus/privnote/issues
[license-shield]: https://img.shields.io/github/license/don-cryptus/privnote.svg?style=for-the-badge
[license-url]: https://github.com/don-cryptus/privnote/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
