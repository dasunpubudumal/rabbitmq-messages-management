import {
  Box,
  AppBar,
  Toolbar,
  Typography,
  IconButton,
} from "@mui/material";
import { Link } from "react-router-dom";

export default function RabbitAppBar() {
  return (
    <>
      <Box>
        <AppBar position="static" color="warning">
          <Toolbar>
            <IconButton
              size="large"
              edge="start"
              color="inherit"
              aria-label="menu"
              sx={{ mr: 2 }}
            >
              <img
                src="favicon.ico"
                alt="Custom Icon"
                style={{ width: 24, height: 24 }}
              />
            </IconButton>
            <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
              <Link to="/" style={{ textDecoration: "none", color: "inherit" }}>
                RabbitMQ Message Viewer
              </Link>
            </Typography>
          </Toolbar>
        </AppBar>
      </Box>
    </>
  );
}
