import init, { generate_julia } from 'drawx';

onmessage = (e) => {
    const data = JSON.parse(e.data);

    let start = Date.now();

    init().then(() => {
      let realPart = data.realPart;
      let imaginaryPart = data.imaginaryPart;

      console.log(`${realPart} + ${imaginaryPart}i`);
      const result = generate_julia(realPart, imaginaryPart, 600, 600);

      let end = Date.now();

      const took = (end - start) / 1000;
      const elapsed = `took: ${took} seconds`;

      const response = {
        loading: false,
        result: result,
        elapsed: elapsed
      };

      postMessage(JSON.stringify(response));
    });
}