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

const profileSchema = {
  properties: {
    id: { type: "string" },
    name: { type: "string" },
    email: { type: "string" },
    profile_pic_url: { type: "string" },
    bio: { type: "string" },
  },
};
const parseProfileResponse = ajv.compileParser(profileSchema);

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

async function profileUpdate({ bio, token }) {
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

const publicProfileSchema = {
  properties: {
    id: { type: "string" },
    name: { type: "string" },
    profile_pic_url: { type: "string" },
    bio: { type: "string" },
  }
};
const postItemSchema = {
  properties: {
    id: { type: "string" },
    post_id: { type: "string" },
    item: { type: "string" },
    quantity: { type: "string" },
  }
}
const postSchema = {
  properties: {
    id: { type: "string" },
    userid: { type: "string" },
    post_type: { enum: ["Needs", "Supplies"] },
    state: { type: "string" },
    district: { type: "string" },
    city: { type: "string" },
    created_at: { type: "timestamp" },
    updated_at: { type: "timestamp" },
    message: { type: "string" },
    items: { elements: { ref: "PostItem" } },
  },
}
const getPostsSchema = {
  properties: {
    posts: { elements: { ref: "Post" } },
    users: { values: { ref: "User" } }
  },
  definitions: {
    "Post": postSchema,
    "User": publicProfileSchema,
    "PostItem": postItemSchema
  }
}
const parseGetPostsResponse = ajv.compileParser(getPostsSchema)

async function getPosts({ start, n, typ }) {
  return await ky.get(BASE_URL + "/posts", {
    searchParams: { start, n, typ },
    parseJson: (text) => {
      const parse = parseGetPostsResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}

const getMyPostsSchema = {
  elements: { ref: "Post" },
  definitions: {
    "Post": postSchema,
    "PostItem": postItemSchema
  }
};
const parseGetMyPostsResponse = ajv.compileParser(getMyPostsSchema)

async function getMyPosts({ token }) {
  return await ky.get(BASE_URL + "/my_posts", {
    headers: {
      "Authorization": "Bearer " + token,
    },
    parseJson: (text) => {
      const parse = parseGetMyPostsResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}

const parseCreatePostResponse = ajv.compileParser({
  ...postSchema,
  definitions: {
    "PostItem": postItemSchema
  }
});
const validatePostItems = ajv.compile({
  properties: {
    item: { type: "string" },
    quantity: { type: "string" },
  }
})
async function createPost({ post_type, state, district, city, message, items, token }) {
  if (!validatePostItems(items)) {
    throw { errors: validatePostItems.errors };
  }

  return await ky.post(BASE_URL + "/posts", {
    headers: {
      "Authorization": "Bearer " + token,
    },
    json: {
      post_type,
      state,
      district,
      city,
      message,
      items
    },
    parseJson: (text) => {
      const parse = parseCreatePostResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}

async function updatePost({ id, post_type, state, district, city, message, items, token }) {
  if (!validatePostItems(items)) {
    throw { errors: validatePostItems.errors };
  }

  return await ky.patch(BASE_URL + "/posts/" + id, {
    headers: {
      "Authorization": "Bearer " + token,
    },
    json: {
      post_type,
      state,
      district,
      city,
      message,
      items
    },
    parseJson: (text) => {
      const parse = parseCreatePostResponse;
      let data = parse(text);
      if (data === undefined) {
        throw { message: parse.message, position: parse.position };
      }
      return data;
    }
  }).json()
}


async function deletePost({ id, token }) {
  return await ky.delete(BASE_URL + "/posts/" + id, {
    headers: {
      "Authorization": "Bearer " + token,
    },
  })
}

export default { login, profile, profileUpdate, getPosts, getMyPosts, createPost, updatePost, deletePost };