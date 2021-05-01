import ky from 'ky';
import Ajv from 'ajv/dist/jtd';
const ajv = new Ajv();

const BASE_URL = "http://localhost:8089"

const parseLoginResponse = ajv.compileParser({
  properties: {
    our_token: { type: "string" }
  },
});

async function login({ token }) {
  return await ky.post(BASE_URL + "/login", {
    json: {
      token
    },
    parseJson: (text) => {
      let data = parseLoginResponse(text);
      if (data === undefined) {
        throw { message: parseLoginResponse.message, position: parseLoginResponse.position };
      }
      return data;
    }
  }).json()
}

export default { login };