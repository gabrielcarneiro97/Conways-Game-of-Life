import * as React from 'react';
import { Rect } from 'react-konva';

import './CellButton.css';

export class CellButton extends React.Component<any, any> {
  constructor (props: any) {
    super(props);
    this.state = { isAlive: false };
  }
  click = (e: any) => {
    this.props.onClick(this);    
    this.setState((prevState: any, props: any) => ({
      isAlive: !prevState.isAlive
    }));
  }
  render () {
    const color = this.state.isAlive ? 'black' : 'white';

    return <Rect
      x={this.props.x}
      y={this.props.y}
      width={this.props.side}
      height={this.props.side}
      fill={color}
      shadowBlur={0}
      onClick={this.click}
      stroke={"black"}
      strokeWidth={.5}
    />
  }
}
