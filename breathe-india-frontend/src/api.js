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

const parseProfileResponse = ajv.compileParser({
  properties: {
    id: { type: "string" },
    name: { type: "string" },
    email: { type: "string" },
    profile_pic_url: { type: "string" },
    bio: { type: "string" },
  },
});

async function profile({ token }) {
  return await ky.get(BASE_URL + "/profile", {
    headers: {
      "Authorization": "Bearer " + token
    },
    parseJson: (text) => {
      const parse = parseProfileResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}

async function profile_update({ bio, token }) {
  return await ky.post(BASE_URL + "/profile", {
    headers: {
      "Authorization": "Bearer " + token,
    },
    json: {
      bio
    },
    parseJson: (text) => {
      const parse = parseProfileResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}

export default { login, profile, profile_update };