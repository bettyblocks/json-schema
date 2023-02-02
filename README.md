# JSON schema

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

## Our workflow

1.  Let your us know what you are working on by creating a JIRA ticket via our Techsupport department.

2.  Branch from `acceptance`.

    ```bash
    $ git checkout acceptance
    $ git pull
    $ git checkout -b feat/a-summary-of-your-ticket-{STORY-ID}
    ```

3.  Work on your feature.

4.  When you're confident about your work, submit a pull request to `edge` and assign it to one of the reviewers. You can comment on your techsupport ticket in Jira **"In review"** and provide the link to the pull request.

    - If there are conflicts, do not merge `edge` into your branch, you can try merging `acceptance` in your branch and else contact techsupport.

5.  Once the ticket is in review our tech department will either give you feedback to make changes or it will be added to the sprint of a team so that the feature can go through our testing process.

6.  Once testing is complete, the techsupport ticket will be promoted to ready for acceptance and you can create a pull request to `acceptance`.

7.  When your work is merged into `acceptance`, you can assume that it will be released with the next release.
