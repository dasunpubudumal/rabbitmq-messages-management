import React, { useState } from "react";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";
import ListItemText from "@mui/material/ListItemText";
import LockOpenIcon from "@mui/icons-material/LockOpen";
import { base64Decode, truncateMessage } from "../../utilities/messages";
import ReusableModal from "../fragments/ReusableModal";
import LabelIcon from "@mui/icons-material/Label";
import { Typography } from "@mui/material";

const style = {
  position: "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: 500,
  bgcolor: "background.paper",
  border: "2px solid #000",
  boxShadow: 24,
  p: 4,
};

export default function Base64EncodedMessageViewer({ messages }) {
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [selectedMessage, setSelectedMessage] = useState("");

  return (
    <>
    {messages && messages.length === 0 && <Typography sx={{ color: "text.secondary" }}>No messages</Typography>}
      <ReusableModal
        open={open}
        handleClose={handleClose}
        style={style}
        title="Decoded message"
        message={base64Decode(selectedMessage)}
      />
      <nav>
        <List>
          {messages &&
            messages.map((message, index) => (
              <ListItem
                disablePadding
                key={index}
                secondaryAction={
                  <Tooltip title="Decode">
                    <IconButton
                      aria-label="comment"
                      onClick={() => {
                        console.log(`Selected message: ${message.payload}`);
                        setSelectedMessage(message.payload);
                        handleOpen();
                      }}
                    >
                      <LockOpenIcon />
                    </IconButton>
                  </Tooltip>
                }
              >
                <ListItemIcon>
                  <LabelIcon />
                </ListItemIcon>
                <ListItemText primary={truncateMessage(message.payload, 40)} />
              </ListItem>
            ))}
        </List>
      </nav>
    </>
  );
}
