# Introduction

Hatsu is a self-hosted application that brings your static site to the Fediverse.

It makes everything as automated as possible:
All you have to do is enter a website URL with a valid feed and set up a redirect, and Hatsu does the rest.

It automatically checks for feed updates to create/update posts
and forwards the received responses back to the site in a Webmention.

## Features

- Written in Rust.
  - It's fast and suitable for running on a low-provisioned VPS.
- Supports both SQLite and Postgres.
  - If you don't want to use Postgres, SQLite would be an easy solution.
  <!-- - Also works well with [Litestream](https://litestream.io/) or [LiteFS](https://fly.io/docs/litefs/)! -->

<!-- ## Comparison

### Bridgy Fed

### Bridgy -->
