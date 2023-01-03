import { ReactNode, useState } from "react";
import ReactDOM from "react-dom/client";
import { Stack } from "@mui/system";
import {
  Backdrop,
  Breakpoint,
  Button,
  CircularProgress,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
} from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import CancelIcon from "@mui/icons-material/Cancel";
import ErrorIcon from "@mui/icons-material/Error";

interface CustomDialogProps {
  title?: string;
  titleIcon?: "none" | "info" | "success" | "error" | "warning";
  content: ReactNode;
  okText?: string;
  okType?: "text" | "outlined" | "contained";
  okColor?: "error" | "inherit" | "primary" | "secondary" | "success" | "info" | "warning";
  onOk?: () => any;
  cancelText?: string;
  cancelType?: "text" | "outlined" | "contained";
  cancelColor?: "error" | "inherit" | "primary" | "secondary" | "success" | "info" | "warning";
  onCancel?: () => any;
  maxWidth?: Breakpoint;
}

const iconReflect = {
  none: "",
  info: <InfoIcon color="info" />,
  success: <CheckCircleIcon color="success" />,
  error: <CancelIcon color="error" />,
  warning: <ErrorIcon color="warning" />,
};

// eslint-disable-next-line @typescript-eslint/no-empty-function
const loop = () => {};

export default function createCustomDialog(props: CustomDialogProps) {
  const {
    title,
    titleIcon = "none",
    content,
    okText = "确定",
    okType = "text",
    okColor = "primary",
    onOk = loop,
    cancelText,
    cancelType = "text",
    cancelColor = "inherit",
    onCancel = loop,
    maxWidth = "xs",
  } = props;

  const node = document.createElement("div");
  document.body.append(node);

  const container = ReactDOM.createRoot(node);

  const CustomDialog = () => {
    const [showBackdrop, setShowBackDrop] = useState<boolean>(false);

    const handleOnClose = () => {
      container.unmount();
      if (node.parentNode) node.parentNode.removeChild(node);
    };

    const handleOnOk = async () => {
      const fn = onOk();
      if (fn instanceof Promise) {
        setShowBackDrop(true);
        await fn;
      }
      container.unmount();
      if (node.parentNode) node.parentNode.removeChild(node);
    };

    const handleOnCancel = async () => {
      const fn = onCancel();
      if (fn instanceof Promise) {
        setShowBackDrop(true);
        await fn;
      }
      container.unmount();
      if (node.parentNode) node.parentNode.removeChild(node);
    };

    return (
      <Backdrop open={showBackdrop} sx={{ color: "#666", zIndex: 9999 }} invisible={true}>
        {showBackdrop && <CircularProgress color="inherit" />}
        <Dialog open fullWidth maxWidth={maxWidth} onClose={handleOnClose}>
          {title && (
            <DialogTitle>
              <Stack direction="row" spacing={1} alignItems="center">
                {iconReflect[titleIcon]}
                <span>{title}</span>
              </Stack>
            </DialogTitle>
          )}
          <DialogContent>{content}</DialogContent>
          <DialogActions>
            {cancelText && (
              <Button variant={cancelType} color={cancelColor} onClick={handleOnCancel}>
                {cancelText}
              </Button>
            )}
            <Button variant={okType} color={okColor} onClick={handleOnOk}>
              {okText}
            </Button>
          </DialogActions>
        </Dialog>
      </Backdrop>
    );
  };

  container.render(<CustomDialog />);
}
