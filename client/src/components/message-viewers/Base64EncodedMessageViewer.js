import React, { useState } from "react";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemIcon from "@mui/material/ListItemIcon";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";
import ListItemText from "@mui/material/ListItemText";
import LockOpenIcon from "@mui/icons-material/LockOpen";
import { base64Decode, truncateMessage } from "../../utilities/messages";
import LabelIcon from "@mui/icons-material/Label";
import { Typography } from "@mui/material";
import { Modal } from "@mui/material";
import { Box } from "@mui/material";
import { downloadMessages } from "../../utilities/messages";
import FileDownloadIcon from "@mui/icons-material/FileDownload";

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

/**
 * Base64EncodedMessageViewer
 *
 * This component displays a list of base64 encoded messages. It allows users to decode
 * and view the full content of a selected message in a modal.
 *
 * @returns {JSX.Element} The rendered component.
 */
export default function Base64EncodedMessageViewer({ messages }) {
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [selectedMessage, setSelectedMessage] = useState("");
  const [decodedMessage, setDecodedMessage] = useState("");

  // TODO: Add don't decode messages twice. Decode it once and have it in state.

  /**
   * DecodedMessageModal
   *
   * This component renders a modal that displays a decoded base64 message.
   * It also provides an option to download the decoded message.
   *
   * @returns {JSX.Element} The rendered modal component.
   */
  const DecodedMessageModal = () => {
    return (
      <>
        <Modal
          open={open}
          onClose={handleClose}
          aria-labelledby="modal-modal-title"
          aria-describedby="modal-modal-description"
        >
          <Box sx={style}>
            <Typography id="modal-modal-title" variant="h6" component="h2">
              Decoded message
            </Typography>
            <Typography id="modal-modal-description" sx={{ mt: 2 }}>
              {decodedMessage}
            </Typography>
            <Box display="flex" justifyContent="flex-end" sx={{ mt: 0 }}>
              <Tooltip title="Download">
                <IconButton
                  size="large"
                  variant="contained"
                  onClick={() =>
                    downloadMessages(decodedMessage)
                  }
                >
                  <FileDownloadIcon color="warning" />
                </IconButton>
              </Tooltip>
            </Box>
          </Box>
        </Modal>
      </>
    );
  };

  return (
    <>
      {messages && messages.length === 0 && (
        <Typography sx={{ color: "text.secondary" }}>No messages</Typography>
      )}
      {messages && messages.length > 0 && (
        <>
          <DecodedMessageModal style={style} />
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
                            setDecodedMessage(base64Decode(message.payload));
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
                    <ListItemText
                      primary={truncateMessage(message.payload, 40)}
                    />
                  </ListItem>
                ))}
            </List>
          </nav>
        </>
      )}
    </>
  );
}
