import * as React from 'react';
import { CellButton } from './CellButton';

export class Map extends React.Component<any, any> {

  constructor (props: any) {
    super(props);
    this.state = { cellStatus : {} }

    for (let i = 0; i < this.props.side; i++) {
      for (let i2 = 0; i2 < this.props.side; i2++) {
        const key = `${i}-${i2}`;
        this.state.cellStatus[key] = false;
      }
    }
  }

  clickCell = (cell: any) => {
    const key = cell._reactInternalFiber.key;
    this.setState((prevState: any) => {
      return {
        cellStatus: { ...prevState.cellStatus, [key]: !prevState.cellStatus[key] }
      }
    }, () => {
      console.log(this.state.cellStatus);      
    });
  }

  render () {
    
    const matrix = [];

    const btnSide = 95 / this.props.side;

    for (let i = 0; i < this.props.side; i++) {
      const line = [];
      for (let i2 = 0; i2 < this.props.side; i2++) {
        const key = `${i}-${i2}`;
        line.push(<CellButton key={key} onClick={this.clickCell} side={btnSide} />);
      }
      matrix.push(
        <div className="line" key={`line-${i}`} style={{height: `${btnSide}vh`}}>
          {line}
        </div>
      );
    }

    return (
      <div>
        {matrix}        
      </div>
    );
  }
}
