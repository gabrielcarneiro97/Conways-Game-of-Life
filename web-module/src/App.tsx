import * as React from 'react';
import './App.css';
import { Map } from './components/Map';
import { SideForm } from './components/SideForm';


class App extends React.Component<any, any> {

  state = { mapSide: 10 }

  newSide = (val: any) => {
    console.log(val);
    this.setState({mapSide: val});
  }

  public render () {
    return (
      <div className="App">
        <SideForm onSubmit={this.newSide} />
        <Map side={this.state.mapSide} />
      </div>
    );
  }
}

export default App;
