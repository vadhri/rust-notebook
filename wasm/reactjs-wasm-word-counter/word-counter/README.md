In order to build the app, please follow the sequence below.

Build the rust package
```cargo build```

Package Wasm - The above command generates the package pkg folder
```wasm-pack build```

Install react dependencies in the word-counter folder. ( adjust the pkg folder as required in package.json )
```npm install```

For development purpose, in the word-counter folder run the command below.
```npm run start```
