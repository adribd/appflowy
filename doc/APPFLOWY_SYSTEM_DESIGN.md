# π₯³ AppFlowy - Event Driven System

* [Goals of the System](#goals-of-the-system)
* [Some Design Considerations](#some-design-Considerations)
* [High Level Design](#high-level-design)
* [Component Design](#component-design)

## π― Goals of the System
The AppFlowy project is an attempt to build a high performance application. Here are the top-level requirements for our system.

1. **High Performance.**
2. **Cross-platform.**
3. **Reliability.**
4. **Safety.**


## π€ Some Design Considerations

## π High Level Design

## π Component Design

### π Event Dispatch

```
                        Frontend                                                     FLowySDK
                                                             β                                              βββββββββββ
                                                             β                                          β7ββΆβHandler Aβ
                                                             β                                          β   βββββββββββ
                                                             β                             βββββββββββ  β   βββββββββββ
ββββββββ    ββββββ    ββββββββββββββββ                       β                        βββββΆβModule A ββββΌβββΆβHandler Bβ
βWidgetββ1ββΆβBlocββ2ββΆβ Repository A ββ3ββ                   β                        β    βββββββββββ  β   βββββββββββ
ββββββββ    ββββββ    ββββββββββββββββ   β                   β                        β                 β   βββββββββββ
                      ββββββββββββββββ   β    βββββββββ    βββ΄βββ     βββββββββββββ   β    βββββββββββ  ββββΆβHandler Cβ
                      β Repository B βββββΌββββΆβ Event ββ4ββΆβFFI ββ5βββΆβDispatcher ββ6ββΌββββΆβModule B β      βββββββββββ
                      ββββββββββββββββ   β    βββββββββ    βββ¬βββ     βββββββββββββ   β    βββββββββββ
                      ββββββββββββββββ   β                   β                        β
                      β Repository C βββββ                   β                        β    βββββββββββ
                      ββββββββββββββββ                       β                        βββββΆβModule C β
                                                             β                             βββββββββββ
                                                             β
                                                             β
```
Here is the event flow:
1. User click on the `Widget`(The user interface) that invokes the `Bloc` actions
2. `Bloc` calls the repositories to perform additional operations to handle the actions.
3. `Repository` offers the functionalities by combining the event, defined in the `FlowySDK`.
4. `Events` will be passed in the `FlowySDK` through the [FFI](https://en.wikipedia.org/wiki/Foreign_function_interface) interface.
5. `Dispatcher` parses the event and generates the specific action scheduled in the `FlowySDK` runtime.
6. `Dispatcher` find the event handler declared by the modules.
7. `Handler` consumes the event and generates the response. The response will be returned to the widget through the `FFI`.

The event flow will be discussed in two parts: the frontend implemented in flutter and the FlowySDK implemented in Rust.

#### FlowySDK



#### Frontend
The Frontend follows the DDD design pattern, you can recap from [**here**](DOMAIN_DRIVEN_DESIGN.md).
```
    ββββββββ        ββββββ        ββββββββββββββββ
    βWidgetβββ1βββββΆβBlocβββ2βββββΆβ Repository A ββ3βββ
    ββββββββ        ββββββ        ββββββββββββββββ    β
                                  ββββββββββββββββ    β     βββββββββ
                                  β Repository B ββββββΌβββββΆβ Event β
                                  ββββββββββββββββ    β     βββββββββ
                                  ββββββββββββββββ    β
                                  β Repository C ββββββ
                                  ββββββββββββββββ
```
