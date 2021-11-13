type EchoData = string;

interface Client {
  echo: (data: string) => Promise<string>;
}

function Client() {
  /*
  Echoes a string
  */


  // TODO: Do not hardcode
  const url = "http://localhost:8000/"

  return {
    echo: function (data: string) {
      return fetch(url + "echo", {
        method: "POST",
        body: data,
      }).then(console.log);
    },
  };
}

export default Client;
