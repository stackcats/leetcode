type Callback = (...args: any[]) => any;
type Subscription = {
  unsubscribe: () => void
}

class EventEmitter {
  events = new Map<string, Callback[]>();
  subscribe(eventName: string, callback: Callback): Subscription {
    if (!(eventName in this.events)) {
      this.events[eventName] = [];
    }
    const listeners = this.events[eventName];
    listeners.push(callback);

    return {
      unsubscribe: () => {
        const index = listeners.indexOf(callback);
        listeners.splice(index, 1);
      }
    };
  }

  emit(eventName: string, args: any[] = []): any[] {
    const listeners = this.events[eventName] || []
    const results = [];
    for (const callback of listeners) {
      results.push(callback(...args));
    }
    return results;
  }
}

/**
 * const emitter = new EventEmitter();
 *
 * // Subscribe to the onClick event with onClickCallback
 * function onClickCallback() { return 99 }
 * const sub = emitter.subscribe('onClick', onClickCallback);
 *
 * emitter.emit('onClick'); // [99]
 * sub.unsubscribe(); // undefined
 * emitter.emit('onClick'); // []
 */
