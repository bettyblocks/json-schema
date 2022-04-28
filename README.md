# json-schema

The JSON schemas for CLI related entities.

## Host schema locally

To host this schema locally, have Node (v14.2.0) installed and run `npm start` from the root of this project.
You can then visit http://localhost:9797/bettyblocks/json-schema/master/schemas/actions/function.json to view the schema in the browser.

## VSC

To use this schema in Visual Studio Code, add the following to the `json.schemas` settings:

```
{
  "fileMatch": [
      "/functions/**/function.json",
  ],
  "url": "https://raw.githubusercontent.com/bettyblocks/json-schema/master/schemas/actions/function.json"
}
```

or

```
{
  "fileMatch": [
      "/functions/**/function.json",
  ],
  "url": "http://localhost:9797/schemas/actions/function.json"
}
```

respectively, depending on whether you want to use the current master version or the locally hosted version.
