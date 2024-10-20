import { Box, AppBar, Toolbar, Typography, IconButton } from "@mui/material";
import { Button } from "@mui/material";
import { Link, useNavigate } from "react-router-dom";

const pages = ["Publish Messages"];

/**
 * RabbitAppBar Component
 * 
 * A functional component that renders an application bar for the RabbitMQ 
 * Message Viewer. The AppBar includes a logo, the application title as a 
 * link to the homepage, and a series of navigation buttons based on a 
 * predefined list of pages.
 * 
 * Features:
 * - Displays a custom logo as an icon button.
 * - Provides a title that links to the home page.
 * - Dynamically generates navigation buttons from an array of pages.
 * 
 * Usage:
 * Import and use the component within a parent component, ensuring that
 * the `pages` array is defined in the parent scope.
 * 
 * Example:
 * 
 * ```jsx
 * import RabbitAppBar from './RabbitAppBar';
 * 
 * function App() {
 *   const pages = ['Home', 'Settings', 'Logs'];
 *   return (
 *     <div>
 *       <RabbitAppBar pages={pages} />
 *       // Other components
 *     </div>
 *   );
 * }
 * ```
 * 
 * @returns {JSX.Element} The rendered AppBar component.
 */
export default function RabbitAppBar() {
  const handleCloseNavMenu = () => {};

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
            {pages.map((page) => (
              <Button
                key={page}
                onClick={handleCloseNavMenu}
                sx={{ my: 2, color: "white", display: "block" }}
              >
                {page}
              </Button>
            ))}
          </Toolbar>
        </AppBar>
      </Box>
    </>
  );
}
