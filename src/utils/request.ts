import axios, { AxiosRequestConfig, AxiosResponse } from "axios";
import createCustomDialog from "@components/create-custom-dialog";
import createCustomAlert from "@components/create-custom-alert";
import { getAuthToken } from "./auth";

const instance = axios.create({
  baseURL: process.env.REACT_APP_BASE_API,
  timeout: 10000,
});

instance.interceptors.request.use(
  (config: AxiosRequestConfig) => {
    config.headers = config.headers || {};
    config.headers["Authorization"] = `Bearer ${getAuthToken()}`;
    config.headers["Content-Type"] = "application/json";
    return config;
  },
  (error) => {
    console.log(error);
    return Promise.reject(error);
  }
);

instance.interceptors.response.use(
  (response: AxiosResponse) => {
    const { data } = response;
    const code = data.code || 404
    if(code === 401) {
      createCustomDialog({
        content: '登录状态已过期，您可以继续留在该页面，或者重新登录',
        okText: '重新登录'
      })
      return false
    } else if(code === 6401) {
      createCustomDialog({
        content: '登录状态已过期，您可以继续留在该页面，或者重新登录',
        okText: '重新登录'
      })
      return false
    } else if(code === 400 || code === 403) {
      createCustomAlert({
        message: data.msg,
        severity: "error",
        duration: 5 * 1000,
      });
      return false
    } else if(code !== 200) {
      createCustomAlert({
        message: data.msg,
        severity: "error",
        duration: 5 * 1000,
      });
      return false
    } else {
      return data;
    }
  },
  (error) => {
    if (error.message === "Network Error") {
      createCustomAlert({
        message: "服务器连接异常，请检查服务器！",
        severity: "error",
        duration: 5 * 1000,
      });
      return;
    }
    if(process.env.NODE_ENV === 'development') {
      console.log(error) // for debug
    }
    return Promise.reject(error);
  }
);

export default instance.request;
