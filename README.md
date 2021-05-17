# Portal

Change Directory Helper.

# Install

You need to run `cargo run --release` currently.

```
git clone git@github.com:shibe23/portal.git
sh shell/install.sh
```

Or setting manually.

```
git clone git@github.com:shibe23/portal.git
echo source `pwd`/shell/portal.sh >> ~/.bashrc
cp `pwd`/target/release/portal /usr/local/bin/portal-exec
```

# Usage

## portal add [label] [dir]

Add `label` and `dir` on `~/.portal` file.

```
portal add dev /path/to/your/directory
```

## portal go [label]

Change your current directory.

```
portal go dev
```

## portal remove [label]

Remove `label` and `dir` from your `~/.portal.json`

```
portal remove dev
```

## portal list

Show `~/.portal`

```
portal list

# dev /path/to/your/directory
```

# How to develop

## run

```
cargo run -- [command] [args]
```

## build to release

```
cargo run --release
```

# LICENCE

MIT
