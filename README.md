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

## Testing

You know it!

```bash
cargo test
```

This will result in a single failing test in the `tests/character.rs` file which has the following error:

```bash
---- test_create_character stdout ----
Running at: http://127.0.0.1:8000
thread 'test_create_character' panicked at 'called `Result::unwrap()` on an `Err` value:
GQLClient Error: Look at json field for more details
Message: Invalid value for argument "character", field "fullName" of type "String!" is required but not provided
', tests/character.rs:38:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This is because, I beieve, that the `reqwest-graphql` is not being respecting the auto-conversion of snakecase to camelcase for some reason. More testing! Going to try various rename options that exist (and that Erik suggested)... Sadly, that didn't work. Check the comments on the GQL type in `main.rs` on lines 44-63.

## Success!...But How?

I'm keeping the above for posterity and explaining how I resolved this. It turns out that `reqwest-graphql` hasn't been actively developed in a year or more nor does it appear to be a public repo I can find. This is a bummer because I could have submitted a PR that would have resolved confusion from the above. It appears that parsing errors on input are logged with details, as the error I call out above, but errors in parsing the response and mapping it to a return type fails silently.

Given the code:

```rust
let response = Client::new("http://127.0.0.1:8000")
        .query_with_vars::<CharacterType, Vars>(query, vars)
        .await
        .unwrap();
```

we're specifically interested in `query_with_vars::<CharacterType, Vars>(query, vars)`. The `CharacterType` is the return type and when the `reqwest-graphql` client attempted to deserialize the response from the GQL server it was failing with `Failed to parse response`. Why? Because queries and mutations return their field names as part of the response. For example, running the example server and executing the `createCharacter` mutation the response is this:

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

The `async-graphql` library offers us a little syntactic sugar and flattens that object a bit by only returing the value of the `data` key:

```json
{
  "createCharacter": {
    "id": "7380a455-0169-4642-ad71-e2559e9b876e",
    "fullName": "Ferris the Crab",
    "description": "Pretty cool little buddy."
  }
}
```

So when I tried to serialize that into the `CharacterType` it failed for a couple reasons. First, I broke best GQL best practices and was attempting to return `CharacterType` rather than defining a [response type](https://www.apollographql.com/docs/technotes/TN0002-schema-naming-conventions/#type-names). Further, I didn't take the mutation field into consideration. I've since refactored the code to take advantage of (at least) GQL best practices and used proper suffixes on my input and response types. Further, I created a new type for the test to represent the response type for testing. All of this appear to happen for free within the running application and it's just `reqwest-graphql` that doesn't have some of the conveniences I expected.

To that end, I created a response `struct`:

```rust
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    create_character: CreateCharacterResponse
}
```

And updated the line of the test that attempts to extract the `id` from the response, changing it from:

```rust
let id = Uuid::parse_str(&response.id).unwrap();
```

...because I assumed `response` here was equivalent to `data.createCharacter` in the example response above (remember, again, I forgot about the mutation field `createCharacter` existing), and changed it to:


```rust
 let id = Uuid::parse_str(&response.create_character.character.id).unwrap();
 ```

 Now, the test passes, the GQL's better formatted and follows best practices, I removed a domain model from the GQL schema (trying to keep things clean), and I have a functional (if not idiotmatic, though I've only written rust for about 10 months so maybe it's on point?) rust GQL server. I hope this is useful to someone, and if [Matt Wright](https://crates.io/users/longfellowone), open source this code. I'd love to help maintain it.
