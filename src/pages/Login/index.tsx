import { useEffect, useState } from "react";
import { Button, Card, Container } from "@mui/material";
import { getCaptchaURL } from "@api/login";
import createCustomDialog from '@components/create-custom-dialog'

const Login = () => {
  const [captcha, setCaptcha] = useState<{imgUrl: string, requestId: string}>({imgUrl: '', requestId: ''});

  useEffect(() => {
    getCaptchaURL().then(({ data, requestId }) => {
      setCaptcha({
        imgUrl: data,
        requestId: requestId
      });
    });
  }, []);

  return (
    <Container maxWidth="sm">
      <Card variant="outlined">
        <div>
          <Button variant="contained" onClick={() => {
            createCustomDialog({
              title: '网络错误',
              titleIcon: 'error',
              content: <div style={{color:'red'}}>网络未连接</div>,
              okColor:'error',
              cancelText: '取消',
              onOk: async () => {
                await new Promise<void>((resolve) => setTimeout(() => {resolve()}, 4000))
              }
            })
          }}>modal add</Button>
          <Button variant="contained">modal remove</Button>
          <img src={captcha.imgUrl} alt="" />
        </div>
      </Card>
    </Container>
  );
};

export default Login;
