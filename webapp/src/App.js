import { useEffect, useMemo, useState } from 'react';

function App() {

  const worker = useMemo(
    () => new Worker(new URL('./worker.js', import.meta.url)),
    []
  );

  const [result, setResult] = useState('');
  const [disableButton, setDisableButton] = useState(false);
  const [elapsed, setElapsed] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (window.Worker) {
      worker.onmessage = (e) => {
        const response = JSON.parse(e.data);

        setResult(response.result);
        setElapsed(response.elapsed);

        if(!response.loading) {
          setLoading(false);
          setDisableButton(false);
        }
      }
    }
  }, [worker]);

  const handleGeneratJulia = async (e) => {
    e.preventDefault();
    setDisableButton(true);
    setLoading(true);
    
    let realPart = Math.random() * 1;
    let imaginaryPart = Math.random() * 1;

    if (window.Worker) {
      const payload = {
        realPart: realPart,
        imaginaryPart: imaginaryPart,
      };

      worker.postMessage(JSON.stringify(payload));
    }
  };

  return (
    <div>
      <header>
        <h3>Julia Set</h3>
        
      </header>
      <div>
        <button onClick={handleGeneratJulia} disabled={disableButton}>Generate</button>
      </div>
      {loading ? <div>processing....</div> : <div></div>}

      {
        result !== '' ? (
          <div>
            <h5>Result</h5>
            <h5>{elapsed}</h5>
            {
              loading ? <div></div> : (
                <div style={{marginBottom: `${25}px`}} >
                <a download="julia.png" href={`data:image/png;base64,${result}`}>
                  <button>Download</button>
                </a>
              </div>
              )
            }
            <img src={`data:image/png;base64,${result}`} alt="Red dot" />
          </div>
        ) : <div></div>
      }
    </div>
  );
}

export default App;
