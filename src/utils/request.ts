import axios from "axios";

const instance = axios.create({
  baseURL: process.env.REACT_APP_BASE_API,
  timeout: 10000,
})

instance.interceptors.response.use(
  function success(response) {
    return response.data
  },
  function error(error) {
    console.log(error)
  }
)


export default instance.request