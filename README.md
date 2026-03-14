# galvyn-template

Opinionated template repository for galvyn projects

## Requirements

Install cargo-generate:

```bash
cargo install -f cargo-generate
```

Also make sure you have `npm` installed

## Usage

```bash
cargo generate -a myOmikron/galvyn-template
```

## Initial configuration

Update the `galvyn.env` to your liking after initializing from template:

```bash
ln -sf galvyn.env .env
cp galvyn.env.template galvyn.env
```

Make an initial commit:

```bash
cd my-project
git add -A && git commit -m "Initial commit"
```

Initialize frontend:

```bash
pushd frontend
npm ci
popd
```
