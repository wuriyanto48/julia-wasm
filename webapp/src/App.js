import { useState } from 'react';
import init, { generate_julia } from 'drawx';

function App() {
  const [result, setResult] = useState('');
  const [disableButton, setDisableButton] = useState(false);
  const [elapsed, setElapsed] = useState('');

  const handleGeneratJulia = async (e) => {
    e.preventDefault();
    setDisableButton(true);
    let start = Date.now();

    init().then(() => {
      let realPart = Math.random() * 1;
      let imaginaryPart = Math.random() * 1;
      console.log(`${realPart} + ${imaginaryPart}i`);
      setResult(generate_julia(realPart, imaginaryPart, 600, 600));

      setDisableButton(false);
      let end = Date.now();

      let took = (end - start) / 1000;
      setElapsed(`took: ${took} seconds`);
    });
    
  };

  return (
    <div>
      <header>
        <h3>Julia Set</h3>
        
      </header>
      <div>
        <button onClick={handleGeneratJulia} disabled={disableButton}>Generate</button>
      </div>

      {
        result !== '' ? (
          <div>
            <h5>Result</h5>
            <h5>{elapsed}</h5>
            <img src={`data:image/png;base64,${result}`} alt="Red dot" />
          </div>
        ) : <div></div>
      }
    </div>
  );
}

export default App;
