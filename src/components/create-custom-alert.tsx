import ReactDOM from "react-dom/client";
import { Alert, AlertColor, Fade, Snackbar } from "@mui/material";
import { useEffect, useState } from "react";

interface CustomDialogAlert {
  message: string;
  severity?: AlertColor;
  duration?: number;
}

export default function createCustomAlert(props: CustomDialogAlert) {
  const { message, severity = "info", duration = 3 * 1000 } = props;

  const node = document.createElement("div");
  document.body.append(node);

  const container = ReactDOM.createRoot(node);

  const removeCustomAlert = () => {
    container.unmount();
    if (node.parentNode) node.parentNode.removeChild(node);
  };

  const CustomAlert = () => {
    const [open, setOpen] = useState<boolean>(true);

    useEffect(() => {
      setTimeout(() => {
        setOpen(false);
        removeCustomAlert();
      }, duration);
    }, []);

    return (
      <Snackbar open={open} anchorOrigin={{ vertical: "top", horizontal: "center" }} TransitionComponent={Fade}>
        <Alert severity={severity}>{message}</Alert>
      </Snackbar>
    );
  };

  container.render(<CustomAlert />);
}
