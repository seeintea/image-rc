import { Button } from "@mui/material";
import { useEffect, useState } from "react";
import { getCaptchaURL } from "@api/login";

const Login = () => {
  const [captcha, setCaptcha] = useState<string>("");

  useEffect(() => {
    getCaptchaURL().then(({ data }) => {
      setCaptcha(data);
    });
  }, []);

  return (
    <div>
      <Button>login</Button>
      <img src={captcha} alt="" />
    </div>
  );
};

export default Login;
