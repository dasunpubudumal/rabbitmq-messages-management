import React, { useState } from "react";
import { useParams } from "react-router-dom";
import Box from "@mui/material/Box";
import Card from "@mui/material/Card";
import CardActions from "@mui/material/CardActions";
import CardContent from "@mui/material/CardContent";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import TextField from "@mui/material/TextField";
import Modal from "@mui/material/Modal";

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

  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  const getMessages = async (count) => {
    if (count === 0) {
      handleOpen();
    } else {
      let response = await fetch(`/queues/${vhost}/${queue}?count=1`);
      setMessages(await response.json());
    }
  };

  const handleCountChange = (event) => {
    setCount(event.target.value);
  };

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
            Text in a modal
          </Typography>
          <Typography id="modal-modal-description" sx={{ mt: 2 }}>
            Duis mollis, est non commodo luctus, nisi erat porttitor ligula.
          </Typography>
        </Box>
      </Modal>
      <Card sx={{ minWidth: 275 }}>
        <CardContent>
          <Typography
            gutterBottom
            sx={{ color: "text.secondary", fontSize: 14 }}
          >
            Queue Details
          </Typography>
          <Typography sx={{ color: "text.secondary", mb: 1.5 }}>
            Name: {queue}
          </Typography>
          <Typography sx={{ color: "text.secondary", mb: 1.5 }}>
            vhost: {vhost}
          </Typography>
        </CardContent>
        <CardActions>
          <Box
            component="form"
            sx={{ "& .MuiTextField-root": { m: 1, width: "25ch" } }}
            noValidate
            autoComplete="off"
          >
            <div>
              <TextField
                required
                id="count"
                type="number"
                label="Number of messages"
                defaultValue="Hello World"
                onChange={handleCountChange}
              />
            </div>
          </Box>
          <Button
            variant="contained"
            onClick={async () => {
              await getMessages(count);
            }}
          >
            Get Messages
          </Button>
        </CardActions>
      </Card>
    </>
  );
}
