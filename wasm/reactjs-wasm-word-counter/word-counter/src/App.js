import React, { useState, useEffect } from 'react';
import {
  MDBInput,
  MDBContainer,
  MDBDataTable
} from 'mdbreact';

import './App.css';

function App() {
  const [text, setText] = useState("");
  const [wasmlib, setWasmlib] = useState(0);
  const [results, setResults] = useState({});

  const loadWasm = async () => {
    try {
      const wasm = await import('wasm-word-counter');
      setWasmlib({wasm});
    } catch(err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  useEffect(() => {
    if (wasmlib == 0) {
      loadWasm();
    }
  });

  const handleChangingText = async (txt) => {
    let ret = wasmlib.wasm.count_words(txt);
    let values = [];

    Object.entries(ret.values).map( o => {
      values.push({
        word: o[0],
        count: o[1]
      })
    });

    console.log(ret.langauge);

    setResults({
      columns: [
        {
          label: ret.langauge + ' Word',
          field: 'word',
          sort: 'asc',
          width: 150
        },
        {
          label: 'count',
          field: 'count',
          sort: 'desc',
          width: 270
        }],
      rows: values
    })
  }

  return (
      <MDBContainer>
        <p className="h2 text-center py-4">How many words was that ?</p>
      <MDBInput className="py-4" type="textarea" label="Please enter text in the area below" rows="5" columns="150" getValue={handleChangingText}/>
      <MDBDataTable
        striped
        bordered
        small
        data={results}
      />
      </MDBContainer>
  );
}

export default App;
