import {
  BrowserRouter,
  Route,
  Routes
} from "react-router-dom";

import Home from '../pages/Home';
import Snapshot from '../pages/Snapshot';

const AppRoutes = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='/snapshot' element={<Snapshot />} />
      </Routes>
    </BrowserRouter>
  )
}

export default AppRoutes;