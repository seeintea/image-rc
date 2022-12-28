import request from "@utils/request";

export async function getCaptchaURL(): Promise<{data: string}> {
  return request({ url: "/api/v1/captcha", method: "get" });
}
