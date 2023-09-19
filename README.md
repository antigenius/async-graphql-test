# Demo App

The purpose of this app is to wrap my feeble, little mind around why my tests aren't working.

## Starting

```bash
cargo run
```

Then open a browser [here](http://127.0.0.1:8000) and try getting a character:

```gql
query GetCharacterQuery {
  character {
    id
    fullName
    description
  }
}
```

This will generate a random ID each time with the same name and description, a la:

```json
{
  "data": {
    "character": {
      "id": "04a57e58-21f4-436c-bae6-f29525eabe43",
      "fullName": "Foo",
      "description": "Bar"
    }
  }
}
```

Same is true of creating a character:

```gql
mutation CreateNewCharacter($input: CreateCharacterInputType) {
  createCharacter(character: $input) {
    id
    fullName
    description
  }
}
```

with the query variables:

```json
{
  "input": {
    "fullName": "Ferris the Crab",
    "description": "Pretty cool little buddy."
  }
}
```

And you'll get the same basic data (but a new ID) each time:

```json
{
  "data": {
    "createCharacter": {
      "id": "7380a455-0169-4642-ad71-e2559e9b876e",
      "fullName": "Ferris the Crab",
      "description": "Pretty cool little buddy."
    }
  }
}
```