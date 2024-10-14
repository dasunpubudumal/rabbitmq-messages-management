import { Typography } from "@mui/material";

export default function StringMessageViewer({ messages }) {
  return (
    <>
      {messages && messages.length === 0 && (
        <Typography sx={{ color: "text.secondary" }}>No messages</Typography>
      )}
    </>
  );
}
