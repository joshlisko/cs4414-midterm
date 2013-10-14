public class deadlock{

    public static void main(String[] args) {

      final String resourceA = "resourceA";
      final String resourceB = "resourceB";

      Thread tOne = new Thread() {
        public void run() {
          synchronized(resourceA){
            try{ 
              Thread.sleep(200); //sleep to make sure the other thread locks the other resource
            } catch (InterruptedException e) {}

            synchronized(resourceB){
            }
          }
        }
      };

      Thread tTwo = new Thread(){
        public void run(){
          synchronized(resourceB){
            try{
          Thread.sleep(200); //sleep to make sure the other thread locks the other resource
          } catch (InterruptedException e){}

            synchronized(resourceA){
            }
          }
        }
      };

      System.out.println("Before we launch threads");
      tOne.start(); 
      tTwo.start();
      System.out.println("Process should quit but it never does");
  }
}