"use client";

import axios, {
  type AxiosError,
  HttpStatusCode,
  type InternalAxiosRequestConfig,
} from "axios";
import { RootPath } from "configs/routes/constants";
import Router from "next/router";

axios.interceptors.request.use(
  async (requestConfig: InternalAxiosRequestConfig) => {
    // try {
    // TODO configure seesion for requests
    // const session: AuthSession = await sessionManager.fetchSession();
    // if (session?.accessToken) {
    //   requestConfig.headers.Authorization = `Bearer ${session.accessToken}`;
    // }
    // } catch (error) {
    //   console.error(error);
    // }

    return requestConfig;
  },
);

axios.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    if (error.response) {
      switch (error.response.status as HttpStatusCode) {
        case HttpStatusCode.Unauthorized:
          // TODO logout await signOut({ redirect: false });
          await Router.push(
            `/${RootPath.SignIn}`,
            `/${RootPath.SignIn}${window.location.search}`,
          );
          break;
        default:
          return Promise.reject(error);
      }
      throw error;
    } else {
      return Promise.reject(error);
    }
  },
);
