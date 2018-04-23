import * as React from 'react';
import './App.css';
import { Map } from './components/Map'

class App extends React.Component {
  public render () {
    return (
      <div className="App">
        <Map />
      </div>
    );
  }
}

export default App;
