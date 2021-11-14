type EchoData = string;

interface Client {
  upload: (data: string) => Promise<string>;
}

function Client() {
  /*
  Echoes a string
  */


  // TODO: Do not hardcode
  const url = "http://localhost:8000/"

  return {
    upload: function (data: Blob) {
      const fileFromBlob = new File([data], "image.png", {
        type: data.type,
      });

      let formData = new FormData();
      formData.append('image', fileFromBlob);

      return fetch(url + "upload", {
        method: "POST",
        body: formData,
      }).then(console.log);
    },
  };
}

export default Client;
