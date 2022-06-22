# Digital Garden

A CLI tool for the creation and maintenance of Digital Gardens

## Commands

### Setting the garden path

```shell
GARDEN_PATH=~/some/path garden write
garden -p ~/some/path write
garden --garden_path ~/some/path write
```

### write

Open a new file to write in our digital garden. Since we
don't necessarily know what we want to title what we're writing,
we'll leave the title as optional and provide a way for users
to specify it if they want

```shell
garden write
garden write -t "Some Title"
```

## Credits

***[Building a Digital Garden CLI with StructOpt](https://rustadventure.dev/building-a-digital-garden-cli-with-structopt) by [Chris Biscardi](https://twitter.com/chrisbiscardi)***
