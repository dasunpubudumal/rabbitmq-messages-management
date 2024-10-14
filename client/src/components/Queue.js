import React, { useState } from "react";
import { useParams } from "react-router-dom";
import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import { filterMessages } from "../utilities/messages";
import MessageViewer from "./message-viewers/MessageViewer";
import ReusableModal from "./fragments/ReusableModal";
import ReusableAlertComponent from "./fragments/ReusableAlertComponent";
import { List, ListItem } from "@mui/material";

const style = {
  position: "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: 400,
  bgcolor: "background.paper",
  border: "2px solid #000",
  boxShadow: 24,
  p: 4,
};

export default function Queue() {
  const [open, setOpen] = useState(false);
  const { vhost, queue } = useParams();
  const [messages, setMessages] = useState([]);
  const [count, setCount] = useState(0);
  const [successAlert, setSuccessAlert] = useState(false);
  const [noMessageAlert, setNoMessageAlert] = useState(false);
  const [jsonMessages, setJsonMessages] = useState([]);
  const [base64Messages, setBase64Messages] = useState([]);
  const [stringMessages, setStringMessages] = useState([]);

  // Modal functions
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  /**
   * Fetches messages from the specified queue.
   *
   * This function fetches messages from the queue based on the provided count.
   * If the count is zero, it opens a modal. Otherwise, it fetches the messages
   * from the queue and updates the state with the fetched messages.
   *
   * @param {number} count - The number of messages to fetch.
   */
  const getMessages = async (count) => {
    setSuccessAlert(false);
    setNoMessageAlert(false);
    if (count === 0) {
      handleOpen();
    } else {
      let response = await fetch(`/queues/${vhost}/${queue}?count=1`);
      let respMessages = await response.json();
      setMessages(respMessages);
      setMessageTypes(respMessages);
    }
  };

  /**
   * Categorizes and sets messages into different state variables based on their encoding.
   *
   * This function processes an array of messages and categorizes them into
   * three groups using the `filterMessages` utility function:
   * - JSON messages: Messages with a payload encoding of "string" and valid JSON.
   * - String messages: Messages with a payload encoding of "string" but invalid JSON.
   * - Base64 messages: Messages with a payload encoding of "base64".
   *
   * The categorized messages are then set into their respective state variables:
   * - `setJsonMessages`: Updates the state with JSON messages.
   * - `setBase64Messages`: Updates the state with base64 messages.
   * - `setStringMessages`: Updates the state with string messages.
   *
   * @param {Array} messages - The array of messages to be filtered and categorized.
   */
  const setMessageTypes = (messages) => {
    const { jsonMessages, stringMessages, base64Messages } =
      filterMessages(messages);
    setJsonMessages(jsonMessages);
    setBase64Messages(base64Messages);
    setStringMessages(stringMessages);
    if (
      jsonMessages.length === 0 &&
      stringMessages.length === 0 &&
      base64Messages.length === 0
    ) {
      setNoMessageAlert(true);
    } else {
      setSuccessAlert(true);
    }
  };

  /**
   * Handles the change event for the count input field.
   *
   * This function updates the state with the new count value entered by the user.
   *
   * @param {Object} event - The event object from the input field.
   */
  const handleCountChange = (event) => {
    setCount(event.target.value);
  };

  return (
    <>
      {successAlert && (
        <ReusableAlertComponent
          severity="success"
          title={`${messages && messages.length} messages received`}
          type="success"
        />
      )}
      {noMessageAlert && (
        <ReusableAlertComponent
          severity="warning"
          title="No messages in the queue"
          type="warning"
        />
      )}
      <ReusableModal
        open={open}
        handleClose={handleClose}
        style={style}
        title="Incorrect message count"
        message="Please provide a message count."
      />
      {/* Card view - Manages user input and statistics of the queue */}
      <Card sx={{ minWidth: 275 }}>
        <CardContent>
          <Typography
            variant="h1"
            gutterBottom
            sx={{ color: "text.primary", fontSize: 18 }}
          >
            Queue Details
          </Typography>
          <List>
            <ListItem>
              <Typography sx={{ color: "text.secondary" }}>
                - Name: {queue}
              </Typography>
            </ListItem>
            <ListItem>
              <Typography sx={{ color: "text.secondary" }}>
                - vhost: {vhost}
              </Typography>
            </ListItem>
          </List>
        </CardContent>
        <CardActions>
          <Box
            component="form"
            sx={{ "& .MuiTextField-root": { m: 1, width: "25ch" } }}
            noValidate
            autoComplete="off"
          >
            <div>
              {/* inputProps is deprecated. Fix it later. */}
              <TextField
                required
                id="count"
                type="number"
                label="Number of messages"
                defaultValue="Hello World"
                onChange={handleCountChange}
                inputProps={{ min: 0 }}
              />
            </div>
          </Box>
          <Button
            variant="contained"
            color="warning"
            onClick={async () => {
              await getMessages(count);
            }}
          >
            Get Messages
          </Button>
        </CardActions>
      </Card>

      {/* Displays messages */}
      <MessageViewer
        jsonMessages={jsonMessages}
        base64EncodedMessages={base64Messages}
        stringMessages={stringMessages}
      />
    </>
  );
}
